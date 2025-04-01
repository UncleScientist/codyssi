mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;
mod problem008;
mod problem009;
mod problem010;
mod problem011;
mod problem012;
mod problem013;
mod problem014;
mod problem015;
mod problem016;
mod problem017;

fn main() {
    let _ = crate::problem017::run();
    let _ = crate::problem016::run();
    let _ = crate::problem015::run();
    let _ = crate::problem014::run();
    let _ = crate::problem013::run();
    let _ = crate::problem012::run();
    let _ = crate::problem011::run();
    let _ = crate::problem010::run();
    let _ = crate::problem009::run();
    let _ = crate::problem008::run();
    let _ = crate::problem007::run();
    let _ = crate::problem006::run();
    let _ = crate::problem005::run();
    let _ = crate::problem004::run();
    let _ = crate::problem003::run();
    let _ = crate::problem002::run();
    let _ = crate::problem001::run();
}

pub fn read_sections<S: AsRef<str>>(
    num: usize,
    title: S,
) -> Result<Vec<Vec<String>>, std::io::Error> {
    let path = if num == 0 {
        "test.txt".to_string()
    } else {
        format!("input/problem{num:03}.txt")
    };
    let data = std::fs::read_to_string(path)?;
    println!("Puzzle {num}: {}", title.as_ref());

    let sections = data.trim().split("\n\n").collect::<Vec<_>>();
    Ok(sections
        .iter()
        .map(|section| {
            section
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect())
}

pub fn read_and_split<S: AsRef<str>>(num: usize, title: S) -> Result<Vec<String>, std::io::Error> {
    let path = if num == 0 {
        "test.txt".to_string()
    } else {
        format!("input/problem{num:03}.txt")
    };
    let data = std::fs::read_to_string(path)?;
    println!("Puzzle {num}: {}", title.as_ref());
    Ok(data
        .trim()
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<_>>())
}
