/* TYPES */
#[derive(Debug)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug,PartialEq)]
/* Points */
pub enum Point {
    Love,
    Fifteen,
    Thirty,
    Forty,
  }

#[derive(Debug,PartialEq)]
pub struct PointsData {
    player_one: Point,
    player_two: Point,
}

#[derive(Debug,PartialEq)]
pub struct FortyData {
    player: Player, /* The player who have forty points */
    other_player_point: Point,
}

#[derive(Debug,PartialEq)]
/* Surely incomplete */
pub enum Score {
    Points(PointsData),
    Forty(FortyData),
    Deuce,
    Advantage(Player),
    Game(Player),
  }

/* IMPELMENT TOOLING TRAITS */
impl ToString for Player {
    fn to_string(&self) -> String {
        match self {
            Player::PlayerOne => String::from("Player 1"),
            Player::PlayerTwo => String::from("Player 2"),
        }
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Clone for Player {
    fn clone(&self) -> Player {
        match self {
            Player::PlayerOne => Player::PlayerOne,
            Player::PlayerTwo => Player::PlayerTwo,
        }
    }
}

/* IMPLEMENT TOOLING FUNCTIONS */
pub fn other_player(p: &Player) -> Player {
    if let Player::PlayerOne = p {
        Player::PlayerTwo
    } else {
        Player::PlayerOne
    }
}

pub fn new_game() -> Score {
    Score::Points(PointsData {
        player_one: Point::Love,
        player_two: Point::Love,
    })
}

/* An exemple how to use option to avoid null values */
pub fn increment_point(point: &mut Point) -> Option<Point> {
    match point {
        Point::Love => Some(Point::Fifteen),
        Point::Fifteen => Some(Point::Thirty),
        Point::Thirty => Some(Point::Forty),
        _ => None, /* Outch ! How int could solve Advantage and End of game ? */
    }
}
/* An exemple how to extract values from Option<T> value*/
pub fn read_from_option_point(op: Option<Point>) -> Point {
    op.unwrap_or(Point::Love)
}

/* IMPELMENT TRANSITIONS */
pub fn score_when_deuce(winner: Player) -> Score {
    Score::Advantage(winner)
}

pub fn score_when_advantage(advantaged_player: Player, winner: Player) -> Score {
    let  the_winner: &Player=&winner;
    if advantaged_player.eq(the_winner)
    {
        score_when_game(advantaged_player)

    }else
    {
        Score::Deuce
    }
}

pub fn score_when_forty(current_at_forty: FortyData, winner: Player) -> Score {
    
    let other_player =other_player(&current_at_forty.player);
   
    let  the_winner: &Player=&winner;
    if current_at_forty.player.eq(the_winner)
    {
        score_when_game(current_at_forty.player)

    } else if current_at_forty.other_player_point == Point::Thirty  && other_player.eq(the_winner)
    {
        Score::Deuce

    } else
    {
        match current_at_forty.player {
            Player::PlayerOne => {
                let point_data=PointsData{

                player_one: Point::Forty,
                player_two: current_at_forty.other_player_point,

                };
                score_when_points(point_data, winner)
            }
            Player::PlayerTwo => {
                let point_data=PointsData{

                player_one: current_at_forty.other_player_point,
                player_two: Point::Forty,

                };
                score_when_points(point_data, winner)
            }
        }   
    }
}
   

pub fn score_when_game(winner: Player) -> Score {
    Score::Game(winner)
}

pub fn score_when_points(point_data: PointsData,winner: Player) -> Score {

    match winner {
        Player::PlayerOne => {
            let mut point_data: PointsData = point_data;
            let increment_point=increment_point(&mut point_data.player_one);
            match increment_point {
                Some(Point::Fifteen) => {point_data.player_one= Point::Fifteen},
                Some(Point::Thirty) => {point_data.player_one= Point::Thirty},
                Some(Point::Forty) => {point_data.player_one= Point::Forty},
                None=> return score_when_deuce(winner),
                _=> panic!("incremetation error"),
            };
            Score::Points(point_data)
        }
        Player::PlayerTwo => {
            let mut point_data: PointsData = point_data;
            let increment_point=increment_point(&mut point_data.player_two);
            match increment_point {
                Some(Point::Fifteen) => point_data.player_two= Point::Fifteen,
                Some(Point::Thirty) => point_data.player_two= Point::Thirty,
                Some(Point::Forty) => point_data.player_two= Point::Forty,
                None=> return score_when_deuce(winner),
                _=> panic!("incremetation error"),
            };
            Score::Points(point_data)
        }
    }
}

pub fn score(current_score: Score, winner: Player) -> Score {
    use tennis::Score::* ;
    match current_score {
    Points(p) => score_when_points(p, winner),
    Forty(f) => score_when_forty(f, winner),
    Deuce => score_when_deuce(winner),
    Advantage(a) => score_when_advantage(a, winner),
    Game(g) => score_when_game(g),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_one_to_string() {
        let p1 = Player::PlayerOne.to_string();
        assert_eq!("Player 1", p1);
    }

    #[test]
    fn player_one_other_player_is_player_two() {
        let p = Player::PlayerOne;
        assert_eq!(Player::PlayerTwo, other_player(&p));
    }

    #[test]
    fn given_deuce_when_player1_wins() {
        let p = Player::PlayerOne;
        assert_eq!( Score::Advantage(Player::PlayerOne), score_when_deuce(p));
    }

    #[test]
    fn given_advantage_when_advantaged_player_wins_then_score_is_game_to_the_advantaged_player() {
        let advantaged_player = Player::PlayerOne;
        let winner = advantaged_player.clone();
        assert_eq!(
            Score::Game(Player::PlayerOne),
            score_when_advantage(advantaged_player, winner)
        );
    }

    #[test]
    fn given_advantage_when_the_other_player_wins_then_score_is_deuce() {
        let advantaged_player = Player::PlayerOne;
        let winner = other_player(&advantaged_player);
        assert_eq!(
            Score::Deuce,
            score_when_advantage(advantaged_player, winner)
        );
    }

    #[test]
    fn given_player_40_when_player_at_40_wins_then_score_is_game_for_this_player() {
        let current_at_forty =  FortyData {
            player: Player::PlayerOne, 
            other_player_point: Point::Forty,
        };
        let winner = Player::PlayerOne;
        assert_eq!(
            Score::Game(Player::PlayerOne),
            score_when_forty(current_at_forty,winner)
        );
    }

    #[test]
    fn given_player_40_other_30_when_other_wins_then_score_is_deuce() {
        let current_at_forty =  FortyData {
            player: Player::PlayerOne, 
            other_player_point: Point::Thirty,
        };
        let winner = Player::PlayerTwo;
        assert_eq!(
            Score::Deuce,
            score_when_forty(current_at_forty,winner)
        );
    }

    #[test]
    fn given_player_40_other_15_when_other_wins_then_score_is_40_30() {
        let current_at_forty =  FortyData {
            player: Player::PlayerOne, 
            other_player_point: Point::Fifteen,
        };
        let winner = Player::PlayerTwo;
        assert_eq!(
            Score::Points(PointsData{
                player_one : Point::Forty,
                player_two : Point::Thirty,
            }),
            score_when_forty(current_at_forty, winner)
        );
    }

    #[test]
    fn given_player_15_other_15_when_player_wins_then_score_is_30_15() {
        let point_data=PointsData{
            player_one: Point::Fifteen,
            player_two: Point::Fifteen,
        } ;
        let winner = Player::PlayerTwo;
        assert_eq!(
            Score::Points(PointsData{
                player_one : Point::Fifteen,
                player_two : Point::Thirty,
            }),
            score_when_points(point_data,winner)
        );
    }

    #[test]
    fn given_player_0_other_15_when_other_wins_then_score_is_0_30() {
        let point_data=PointsData{
            player_one: Point::Love,
            player_two: Point::Fifteen,
        } ;
        let winner = Player::PlayerTwo;
        assert_eq!(
            Score::Points(PointsData{
                player_one : Point::Love,
                player_two : Point::Thirty,
            }),
            score_when_points(point_data,winner)
        );
    }

    #[test]
    fn given_player_30_other_15_when_player_wins_then_score_is_40_15() {
        let point_data=PointsData{
            player_one: Point::Thirty,
            player_two: Point::Fifteen,
        } ;
        let winner = Player::PlayerOne;
        assert_eq!(
            Score::Points(PointsData{
                player_one : Point::Forty,
                player_two : Point::Fifteen,
            }),
            score_when_points(point_data,winner)
        );
    }
}
