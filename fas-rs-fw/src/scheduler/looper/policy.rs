/* Copyright 2023 shadow3aaa@gitbub.com
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License. */
use std::time::{Duration, Instant};

use log::debug;

use super::{Buffer, Looper};
use crate::{error::Result, Config, PerformanceController};

impl<P: PerformanceController> Looper<P> {
    pub fn do_policy(buffer: &mut Buffer, controller: &P, config: &Config) -> Result<()> {
        let Some(target_fps) = buffer.target_fps else {
            return Ok(());
        };
        let window = buffer.windows.get_mut(&target_fps).unwrap();

        let policy = Self::policy_config(config)?;
        debug!("mode policy: {policy:?}");

        let Some(normalized_frametime) = window.get_avg() else {
            return Ok(());
        };

        let normalized_big_jank_scale = calculate_normalized_scale(target_fps, 10.0);
        let normalized_jank_scale = calculate_normalized_scale(target_fps, 3.0);
        /* let normalized_limit_scale =
            calculate_normalized_scale(target_fps, policy.tolerant_frame_limit);
        let normalized_release_scale =
            calculate_normalized_scale(target_fps, policy.tolerant_frame_jank); */

        debug!("target_fps: {target_fps}");
        debug!("normalized frametime: {normalized_frametime:?}");
        debug!("simple jank scale: {normalized_jank_scale:?}");
        debug!("big jank scale: {normalized_big_jank_scale:?}");
        // debug!("limit scale: {normalized_limit_scale:?}");
        // debug!("release scale: {normalized_release_scale:?}");

        if normalized_frametime > normalized_big_jank_scale {
            controller.release_max(config)?; // big jank
            buffer.counter = policy.jank_rec_count;
            debug!("JANK: big jank");
        } else if normalized_frametime > normalized_jank_scale {
            buffer.counter = policy.jank_rec_count;
            if let Some(front) = buffer.frametimes.front_mut() {
                *front = Duration::from_secs(1) / target_fps;
            }

            if let Some(front) = window.frametimes.front_mut() {
                *front = Duration::from_secs(1);
            }

            controller.release(config)?;
            debug!("JANK: simp jank");
        } else if normalized_frametime <= Duration::from_secs(1) {
            if buffer.counter != 0 {
                buffer.counter -= 1;
                return Ok(());
            }

            if let Some(last_limit) = buffer.last_limit {
                let normalized_limit = last_limit.elapsed() * target_fps;
                if normalized_limit <= Duration::from_secs(3) {
                    return Ok(());
                } // 1 limit is allowed every 3 frames
            }

            buffer.last_limit = Some(Instant::now());
            buffer.counter = policy.normal_rec_count;

            controller.limit(config)?;
            debug!("JANK: no jank");
        } else if normalized_frametime > Duration::from_millis(1100) {
            controller.release(config)?;
            debug!("JANK: unit jank");
        }

        Ok(())
    }
}

fn calculate_normalized_scale(t: u32, d: f64) -> Duration {
    Duration::from_secs(1).div_f64((f64::from(t) - d).max(1.0)) * t
}
