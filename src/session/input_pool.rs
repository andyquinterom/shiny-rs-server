pub struct InputPool {
    pub pool: std::collections::HashMap<String, serde_json::Value>
}

impl InputPool {
    pub fn new() -> InputPool {
        InputPool {
            pool: std::collections::HashMap::<String, serde_json::Value>::new()
        }
    }
    pub fn contains(&self, key: &str) -> bool {
        return self.pool.contains_key(key)
    }
    pub fn insert(&mut self, key: &str, value: serde_json::Value) {
        self.pool.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Result<&serde_json::Value, Box<dyn std::error::Error>> {
        let val: &serde_json::Value =  self.pool.get(key).ok_or("Value not stored in inputs")?;
        return Ok(val)
    }
    pub fn get_string(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let val_res = self.get(key)?
            .as_str()
            .ok_or("Value could not be parsed")?
            .to_string();
        return Ok(val_res)
    }
    pub fn get_u64(&self, key: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let val_res = self.get(key)?;
        let val_u64 = val_res.as_u64().ok_or("Value could not be converted to u64")?;
        return Ok(val_u64)
    }
    pub fn get_f64(&self, key: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let val_res = self.get(key)?;
        let val_f64 = val_res.as_f64().ok_or("Value could not be converted to f64")?;
        return Ok(val_f64)
    }
    pub fn get_i64(&self, key: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let val_res = self.get(key)?;
        let val_i64 = val_res.as_i64().ok_or("Value could not be converted to i64")?;
        return Ok(val_i64)
    }
}


