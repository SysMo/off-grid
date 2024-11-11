use morse::outputs::writers::postgress::PostgresWriterConfig;
use morse::processors::current_state_processor::StateProcessorConfig;
use morse::processors::historical_data_processor::{HistoricalDataBatch, HistoricalProcessorConfig};
use morse::processors::simple_transformer::{SimpleTranformer, SimpleTransformerConfig};
use morse::processors::splitter::DataSplitterConfig;
use morse::processors::{DataSplitter, HistoricalProcessor, StateProcessor};
use serde::Deserialize;

use morse::inputs::{Input, InputConfig};
use morse::outputs::{Output, OutputConfig};
use morse::inputs::readers::{MqttReader, MqttReaderConfig};
use morse::data::data_point::RegisterSnapshot;
use morse::outputs::writers::{InfluxDbWriter, InfluxDbWriterConfig, PostgresWriter};
use morse::agent::TAsyncSystem;

use crate::input::MbOffgridType;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OffgridSystemConfig {
  mqtt: InputConfig<MqttReaderConfig>,
  // splitter: DataSplitterConfig,
  // state_processor: StateProcessorConfig<PffStateProcessorConfig>,
  // db_current_state: OutputConfig<PostgresWriterConfig>,
  historical_processor: HistoricalProcessorConfig,
  // event_processor: SimpleTransformerConfig<EventProcessorConfig>,
  db_historical: OutputConfig<InfluxDbWriterConfig>
}

pub struct OffgridSystem {
  mqtt: Input<RegisterSnapshot<MbOffgridType>, MqttReader>,  
  // splitter: DataSplitter<RegisterSnapshot<MbPffType>, 2>,

  // state_processor: StateProcessor<PffStateProcessor>,
  // db_current_state: Output<PffCurrentStateUpdate, PostgresWriter>,

  historical_processor: HistoricalProcessor<MbOffgridType>,
  // event_processor: SimpleTranformer<EventProcessor>,
  db_historical: Output<HistoricalDataBatch, InfluxDbWriter>
}

impl TAsyncSystem for OffgridSystem {
    type Config = OffgridSystemConfig;

    async fn try_from_config(config: Self::Config) -> anyhow::Result<Self> {
      let db_historical = Output::try_from_config(config.db_historical).await?;
      
      let historical_processor = HistoricalProcessor::<MbOffgridType>::try_from_config(
        config.historical_processor, db_historical.data_dispatcher()
      ).await?;
      
      let mqtt = Input::try_from_config(config.mqtt, historical_processor.data_dispatcher()).await?;

      Ok(Self { mqtt, historical_processor, db_historical })
    }

    async fn on_tick(&mut self) -> anyhow::Result<()> {
      self.mqtt.agent.ensure_running().await;
      // self.splitter.agent.ensure_running().await;
      // self.state_processor.agent.ensure_running().await;
      // self.db_current_state.agent.ensure_running().await;
      self.historical_processor.agent.ensure_running().await;
      // self.event_processor.agent.ensure_running().await;
      self.db_historical.agent.ensure_running().await;

      Ok(())
    }
}