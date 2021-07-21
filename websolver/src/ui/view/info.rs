use webelements::{we_builder, Result, WebElement};

use crate::ui::controller::app::AppController;
use crate::ui::controller::info::InfoController;
use crate::ui::model::info::Stat;
use crate::util::InitCell;

#[we_builder(
    <div class="solve-info">
        <InfoStat we_field="tech" we_element />
        <InfoStat we_field="steps" we_element />
        <InfoStat we_field="guesses" we_element />
        <InfoStat we_field="g_steps" we_element />
        <InfoStat we_field="g_total" we_element />
    </div>
)]
#[derive(Debug, Clone)]
pub struct Info {}

impl WebElement for Info {
    fn init(&mut self) -> Result<()> {
        self.tech.stat = Stat::Tech;
        self.steps.stat = Stat::Steps;
        self.guesses.stat = Stat::Guesses;
        self.g_steps.stat = Stat::GSteps;
        self.g_total.stat = Stat::GTotal;

        Ok(())
    }
}

impl Info {
    pub fn controller(&self, app: InitCell<AppController>) -> Result<InfoController> {
        InfoController::build(app, self)
    }

    pub fn update(&self, info: &InfoController) -> Result<()> {
        self.tech.update(info)?;
        self.steps.update(info)?;
        self.guesses.update(info)?;
        self.g_steps.update(info)?;
        self.g_total.update(info)?;
        Ok(())
    }
}

#[we_builder(
    <div class="solve-stat">
        <span class="info-label" we_field="label" />
        <span class="info-value" we_field="value" />
    </div>
)]
#[derive(Debug, Clone, WebElement)]
pub struct InfoStat {
    stat: Stat,
}

impl InfoStat {
    pub fn update(&self, info: &InfoController) -> Result<()> {
        let info = info.info.lock().unwrap();
        let stat = match self.stat {
            Stat::Tech => "Tech",
            Stat::Steps => "Steps",
            Stat::Guesses => "Guesses",
            Stat::GSteps => "Total Steps",
            Stat::GTotal => "Total Guesses",
            _ => "N/A",
        };
        self.label.set_text(format!("{}:", stat));
        if let Some(value) = info.property(self.stat) {
            self.value.set_text(&value);
        }
        Ok(())
    }
}
