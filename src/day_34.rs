#[allow(unused)]
pub mod tournament {

  use std::cmp::Ordering;
  use std::collections::HashMap;

  #[derive(Eq, Debug, PartialOrd, Clone)]
  pub struct Team {
    name: String,
    mp: i32,
    w: i32,
    d: i32,
    l: i32,
    p: i32,
  }

  impl Team {
    fn new(name: &str) -> Self {
      Self {
        name: String::from(name),
        mp: 0,
        w: 0,
        d: 0,
        l: 0,
        p: 0,
      }
    }

    fn win(&mut self) {
      self.mp += 1;
      self.w += 1;
      self.p += 3;
    }

    fn loss(&mut self) {
      self.mp += 1;
      self.l += 1;
    }

    fn draw(&mut self) {
      self.mp += 1;
      self.d += 1;
      self.p += 1;
    }

    fn row(&self) -> String {
      format!(
        "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
        self.name, self.mp, self.w, self.d, self.l, self.p
      )
    }
  }

  impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
      self.name == other.name
    }
  }

  pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<&str, Team> = HashMap::new();

    for m in match_results.split('\n') {
      let mut match_info = m.split(';');

      match (
        match_info.next(),
        match_info.next(),
        match_info.next(),
        match_info.next(),
      ) {
        (Some(first), Some(second), Some(result), _) => match result {
          "win" => {
            teams.entry(first).or_insert(Team::new(first)).win();
            teams.entry(second).or_insert(Team::new(second)).loss();
          }
          "loss" => {
            teams.entry(first).or_insert(Team::new(first)).loss();
            teams.entry(second).or_insert(Team::new(second)).win();
          }
          "draw" => {
            teams.entry(first).or_insert(Team::new(first)).draw();
            teams.entry(second).or_insert(Team::new(second)).draw();
          }
          _ => {}
        },
        _ => {}
      };
    }

    let mut teams_report = teams.values().cloned().collect::<Vec<Team>>();

    teams_report.sort_by(|a, b| match (b.p).cmp(&a.p) {
      Ordering::Equal => (a.name).partial_cmp(&b.name).unwrap(),
      comp => comp,
    });

    println!("{:?}", teams_report);

    let mut table = vec![get_header()];

    for team in teams_report {
      table.push(team.row());
    }

    table.join("\n")
  }

  fn get_header() -> String {
    format!("{:31}| MP |  W |  D |  L |  P", "Team")
  }
}
