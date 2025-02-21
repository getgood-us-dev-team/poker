

struct Player {
    money: u32,
    hand: Vec<Card>,
    position: usize,
    raised_this_round: u32,
}

enum Action {
    Check,
    Fold,
    Call,
    Raise(u32),
}

impl Action {
    fn compile_action(&self) -> String {
        match self {
            Action::Check => "c".to_string(),
            Action::Fold => "f".to_string(),
            Action::Call => "ca".to_string(),
            Action::Raise(amount) => format!("r-{}", amount),
        }
    }
}

impl From<String> for Action {
    fn from(s: String) -> Self {
        match s.as_str() {
            "c" => Action::Check,
            "f" => Action::Fold,
            "ca" => Action::Call,
            _ => {
                if s.starts_with("r-") {
                    let amount = s[2..].parse::<u32>().unwrap();
                    Action::Raise(amount)
                } else {
                    panic!("Invalid action");
                }
            }
        }
    }
}

struct CardServer{
    pipeline: Vec<Action>,
    players: Vec<Player>,
    deck: Deck,
    pot: u32,
    current_player: usize,
    max_raise_this_round: u32,
}

impl CardServer {
    fn new(){
        let mut deck = Deck::new_empty();
        deck.shuffle();
        let pipeline = Vec::new();
        let mut card_server = CardServer {
            pipeline,
            players,
            deck,
            pot: 0,
            current_player: 0,
            max_raise_this_round: 0,
        };
        card_server.deal();
        card_server.play();
    }
}