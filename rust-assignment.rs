// by Rostislav Litovkin
// The code was primarily optimised for great readability ^^

fn main() {
    let stdin = std::io::stdin();

    // Initial input
    let mut largest = 0;
    let mut questions: Vec<u32> = vec![]; 

    let mut length = String::new();
    stdin.read_line(&mut length).expect("Couldn’t read from stdin");
    let length: u32 = length.trim().parse().expect("not an integer");

    for _i in 0..length{

        let mut question = String::new();
        stdin.read_line(&mut question).expect("Couldn’t read from stdin");
        let question: u32 = question.trim().parse().expect("not an integer");

        if question > largest { largest = question; }

        questions.push(question);
    }

    // Dynamically recording all of the answers in between, that can be eventually repeated.
    let mut fourth: u32 = 1; // fourth meaning fourth last
    let mut third: u32 = 2; // third last etc...
    let mut second: u32 = 3;
    let mut first: u32 = 5; 
    let mut results: Vec<u32> = vec![0, 1, 3, 6, 11]; // 0 added for more human readable experience (basically to shift the index to start with number 1)

    for i in 4..largest {

        let temp = ((first + second) % (u32::pow(10, 9) + 7) + fourth) % (u32::pow(10, 9) + 7) ;

        fourth = third;
        third = second;
        second = first;
        first = temp;

        results.push((results[(i as usize)] + temp) % (u32::pow(10, 9) + 7))
    }

    // printing the results
    for i in questions{
        println!("{}", results[i as usize]);
    }
}
