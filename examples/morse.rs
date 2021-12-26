#[derive(Debug, Clone, Copy)]
enum SIGNALS {
    E, I, S, U, A, R, W, T, N, D, K, M, G, O
}
use SIGNALS::*;
impl SIGNALS {
    fn find_matching(signal_str: &str) -> Vec<Self> {
        let sig_enums = [E, I, S, U, A, R, W, T, N, D, K, M, G, O];
        let mut res: Vec<Self> = vec![E; 0];
        for item in sig_enums.iter().enumerate() {
            let (_, sig) = item;
            if sig.is_match_signal(signal_str) == true {
                res.push(*sig);
            }
        }
        res
    }
    fn to_signal_str(&self) -> &str {
        let sigs = [".", "..", "...", "..-", ".-", ".-.", ".--", "-", "-.", "-..", "-.-", "--", "--.", "---" ];
        &sigs[*self as usize]
    }
    fn is_match_signal(&self, sig: &str) -> bool {
        let mysig = self.to_signal_str();
        if mysig.len() == sig.len() {
            let mut flag = 0;
            for item in mysig.as_bytes().iter().enumerate() {
                let (i, ch) = item;
                let sigch = sig.as_bytes()[i];
                if *ch != sigch && sigch != "?".as_bytes()[0] {
                    flag = 1;
                    break;
                }
            }
            if flag == 1 {
                return false;
            }
            return true;
        }
        false
    }
}

fn possibilities(signal: &str) -> Vec<SIGNALS> {
    return SIGNALS::find_matching(signal);
}

fn main() {
  [".", ".-", "?", "?.", ".?", "?-?", "?.?"]
    .iter()
    .enumerate()
    .for_each(|(idx, &code)| {
      let result = possibilities(code);
      println!("Result{0} = {1:?}", idx, result);    
    });
}