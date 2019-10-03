//! Query Representation

use models::api::search::ascdesc::AscDesc;

#[derive(Clone)]
pub struct Query {
    pub select_v: Vec<String>,
    pub from_v: Vec<String>,
    pub where_v: Vec<String>,
    pub join_v: Vec<String>,
    pub order_v: Vec<String>,

    pub limit: i64,
    pub offset: i64,
    pub format: String,
    pub sort_order: AscDesc,
}

impl Query {
    pub fn format(&mut self) -> String {
        if self.where_v.len() == 0 {
            self.where_v.push(String::from("1=1"));
        }

        let select_string = self.select_v.join(",\n");
        let from_string = self.from_v.join(",\n");
        let where_string = self.where_v.join(" AND \n");
        let join_string = self.join_v.join("\n");
        let order_string = match self.order_v.len() {
            0 => String::from(""),
            _ => format!(
                "ORDER BY {} {}",
                self.order_v.join(",\n"),
                self.sort_order.as_str()
            ),
        };

            ;
        let limit_string = format!("LIMIT {} OFFSET {}", self.limit, self.offset);

        return rt_format!(
            self.format,
            select_string,
            rt_format!(from_string, limit_string).unwrap(),
            join_string,
            where_string,
            order_string,
            limit_string
        )
        .unwrap();
    }
}
