use morse::data::IdValue;
use morse::processors::THistoricalDataSource;

use crate::input::MbOffgridType;

//###########################
//## Historical data
//###########################

impl THistoricalDataSource for MbOffgridType {
  fn historical_data(&self) -> Vec<IdValue> {    
    match self {
      MbOffgridType::DTS777Electricity(x) => vec![
        IdValue::f32("v1", x.v1),
        IdValue::f32("v2", x.v2),
        IdValue::f32("v3", x.v3),
        
        IdValue::f32("i1", x.i1),
        IdValue::f32("i2", x.i2),
        IdValue::f32("i3", x.i3),
        
        IdValue::f32("p1", x.p1),
        IdValue::f32("p2", x.p2),
        IdValue::f32("p3", x.p3),
        // IdValue::f32("p_reactive_ph1", x.p_reactive_ph1),
        // IdValue::f32("p_reactive_ph2", x.p_reactive_ph2),
        // IdValue::f32("p_reactive_ph3", x.p_reactive_ph3),
        // IdValue::f32("p_apparent_ph1", x.p_apparent_ph1),
        // IdValue::f32("p_apparent_ph2", x.p_apparent_ph2),
        // IdValue::f32("p_apparent_ph3", x.p_apparent_ph3),
        IdValue::f32("pf1", x.pf1),
        IdValue::f32("pf2", x.pf2),
        IdValue::f32("pf3", x.pf3),

        IdValue::f32("f", x.f),
      
      ],
      MbOffgridType::DTS777Energy(x) => vec![
      //   IdValue::f32("total_energy_demand_active", x.total_energy_demand_active),
      //   IdValue::f32("total_energy_demand_reactive", x.total_energy_demand_reactive),
      //   IdValue::f32("total_energy_apparent", x.total_energy_apparent),
      //   IdValue::f32("total_energy_delivery_active", x.total_energy_delivery_active),
      //   IdValue::f32("total_energy_delivery_reactive", x.total_energy_delivery_reactive),
      //   IdValue::f32("total_energy_reactive", x.total_energy_reactive),
      //   IdValue::f32("current_energy_demand_active", x.current_energy_demand_active),
      //   IdValue::f32("current_energy_demand_reactive", x.current_energy_demand_reactive),
      //   IdValue::f32("current_energy_apparent", x.current_energy_apparent),
      //   IdValue::f32("current_energy_delivery_active", x.current_energy_delivery_active),
      //   IdValue::f32("current_energy_delivery_reactive", x.current_energy_delivery_reactive),
      //   IdValue::f32("current_energy_reactive", x.current_energy_reactive),
      ],
    }
  }
}