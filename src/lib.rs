// Different kind of actions to happen across games
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Action {
   SuccessCardToTable,
   FailCardToTable,
   WonGame,
   LostGame,
   AbanonedGame,
}

pub trait Metric<T> {
    fn get_data (self) -> (String, Action, T, u32);
    fn save(self);
}

// Struct defined to create the rankings
pub struct PlayerWithValue {
    player_id: String,
    value: isize,
}

// Ranking created from metric aggregations, 
pub struct Ranking {
    name: String,
    list: Vec<PlayerWithValue>,
    affected_actions: Vec<Action>,
    date_range: (u32, u32),
}

pub trait RankingMethods<T> {
    fn update(&mut self, metric: &impl Metric<T>);
    fn get_rank(&self) -> Vec<PlayerWithValue>;
}


/* Specfic metric arriving the system, constraints to use this metric are:
    - Value must be a number
    - Metadata field should have a timestamp
*/
pub struct NumMetric {
    player_id: String,
    action: Action,
    value: u32,
    meta: Box<dyn WithTimestamp>,
}

trait WithTimestamp {
    fn get_timestamp(&self) -> u32;
}

impl Metric<u32> for NumMetric {
    fn get_data (self) -> (String, Action, u32, u32) {
        return (self.player_id, self.action, self.value, self.meta.get_timestamp());
    }
    fn save (self) {
        //TBD, save the metric in database
    }
}

struct WonGameMeta {
    date: u32,
    participants: Vec<String>,
    results: Vec<(String, u32)>
}

impl WithTimestamp for  WonGameMeta {
    fn get_timestamp(&self) -> u32 {
       return self.date;
    }
} 


struct SuccessCardToTableMeta {
    date: u32,
    participants: Vec<String>,
    partial_results: Vec<(String, u32)>
}

impl WithTimestamp for  SuccessCardToTableMeta {
    fn get_timestamp(&self) -> u32 {
        return self.date;
    }
} 

impl NumMetric {
    pub fn create_won_game( player_id: String, date: u32, participants: Vec<String>, results: Vec<(String, u32)>) -> NumMetric {
        NumMetric {
            player_id: player_id,
            action: Action::WonGame,
            value: 1,
            meta: Box::new(WonGameMeta {
                date: date,
                participants: participants,
                results: results,
            }),
        }
    }
    pub fn create_success_card_to_table( player_id: String, date: u32, participants: Vec<String>, partial_results: Vec<(String, u32)>) -> NumMetric {
        NumMetric {
            player_id: player_id,
            action: Action::SuccessCardToTable,
            value: 1,
            meta: Box::new(SuccessCardToTableMeta {
                date: date,
                participants: participants,
                partial_results: partial_results,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_create_new_won_game() {
        let mock_users_list = vec![String::from("testId"), String::from("testId1")];
        let mock_results_list = vec![(String::from("testId"), 32), (String::from("testId1"), 24)];
        let won_game_event = NumMetric::create_won_game(String::from("testId"), 12345,  mock_users_list, mock_results_list );
        let view_new_game_event = won_game_event.get_data();
        assert_eq!(view_new_game_event.0, String::from("testId"));
        assert_eq!(view_new_game_event.1, Action::WonGame);
    }
    #[test]

    fn can_create_new_success_card_to_table() {
        let mock_users_list = vec![String::from("testId"), String::from("testId1")];
        let mock_results_list = vec![(String::from("testId"), 32), (String::from("testId1"), 24)];
        let success_card_to_table_event = NumMetric::create_success_card_to_table(String::from("testId"), 12345,  mock_users_list, mock_results_list );
        let view_new_game_event = success_card_to_table_event.get_data();
        assert_eq!(view_new_game_event.0, String::from("testId"));
        assert_eq!(view_new_game_event.1, Action::SuccessCardToTable);
    }
}
