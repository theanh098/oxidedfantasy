use chrono::Utc;
use cron::Schedule;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use tokio::time::sleep;

pub enum CronExpression {
    EveryFiveMinutes,
    EveryThreeMinutes,
}

impl ToString for CronExpression {
    fn to_string(&self) -> String {
        use CronExpression::*;
        match self {
            EveryFiveMinutes => "0 */5 * * * *".into(),
            EveryThreeMinutes => "0 */3 * * * *".into(),
        }
    }
}

pub struct Scheduler<C: 'static + Sized + Default + Clone> {
    context: Option<C>,
    jobs: Vec<(
        String,
        &'static dyn Fn(C) -> Pin<Box<dyn Future<Output = ()>>>,
    )>,
}

impl<'a, C: 'static + Default + Clone> Scheduler<C> {
    pub fn new() -> Self {
        Self {
            context: None,
            jobs: Vec::new(),
        }
    }

    pub fn add_job(
        mut self,
        cron_expression: CronExpression,
        job: &'static impl Fn(C) -> Pin<Box<dyn Future<Output = ()>>>,
    ) -> Self {
        self.jobs.push((cron_expression.to_string(), job));
        self
    }

    pub fn set_context(mut self, context: C) -> Self {
        self.context = Some(context);
        self
    }

    /// Start the execution of the scheduled jobs
    pub async fn start(self) -> Result<(), Box<dyn Error>> {
        // Set up schedules

        let context: C = self.context.unwrap_or_default();

        let mut futures = Vec::new();

        for (cron, job) in self.jobs {
            let con = context.clone();
            let schedule = Schedule::from_str(&cron)?;

            let fut = async move {
                for datetime in schedule.upcoming(Utc) {
                    let now = Utc::now();

                    if let Ok(duration) = datetime.signed_duration_since(now).to_std() {
                        sleep(duration).await;
                        let _result = job(con.clone()).await;
                    }
                }
            };

            futures.push(fut);
        }

        futures::future::join_all(futures).await;

        Ok(())
    }
}
