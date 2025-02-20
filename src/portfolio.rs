use crate::models::{Asset, Portfolio, AssetType};
use std::collections::HashMap;
use uuid::Uuid;

impl Portfolio {
    pub fn new(name: String, initial_balance: f64) -> Self {
        Portfolio {
            id: Uuid::new_v4().to_string(),
            name,
            assets: HashMap::new(),
            cash_balance: initial_balance,
        }
    }

    pub fn buy_asset(&mut self, symbol: String, quantity: f64, price: f64) -> Result<(), String> {
        let total_cost = quantity * price;

        if self.cash_balance < total_cost {
            return Err("Insufficient balance".to_string());
        }

        let asset = self.assets.entry(symbol.clone()).or_insert(Asset {
            id: Uuid::new_v4().to_string(),
            symbol,
            quantity: 0.0,
            purchase_price: price,
            asset_type: AssetType::Stock,
            current_price: 0.0,
        });

        asset.quantity += quantity;
        self.cash_balance -= total_cost;
        Ok(())
    }

    pub fn sell_asset(&mut self, symbol: &str, quantity: f64, price: f64) -> Result<(), String> {
        if let Some(asset) = self.assets.get_mut(symbol) {
            if asset.quantity < quantity {
                return Err("Insufficient quantity".to_string());
            }

            asset.quantity -= quantity;
            self.cash_balance += quantity * price;

            if asset.quantity == 0.0 {
                self.assets.remove(symbol);
            }
            Ok(())
        } else {
            Err("Asset not found".to_string())
        }
    }
}