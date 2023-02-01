use std::io;

fn main() {
    println!("Digite a quantidade de alunos: ");
    let mut total_average_str = String::new();
    io::stdin().read_line(&mut total_average_str).expect("Houve um erro ao ler a quantidade de medias.");
    let total_average = convert_to_i32(&total_average_str);
    
    let mut recovery_sum = 0;
    let mut i = 0;
    while total_average > i {
        println!("Digita a nota do aluno {}: ", i+1);
        let mut student_average = String::new();
        io::stdin().read_line(&mut student_average).expect("Houve um erro ao ler media do aluno");
        i+=1;
        
        if convert_to_i32(&student_average) < 6 {
            recovery_sum += 1;
        }
    }
    
    send_students_statistcs(total_average, recovery_sum);
}

fn send_students_statistcs(total_average: i32, recovery_sum: i32) {
    let students_approved = total_average - recovery_sum;
    let approved_percentage: f32 = calc_percentage(students_approved, total_average);
    let reproved_percentage: f32 = calc_percentage(recovery_sum, total_average);

    println!("ESTATÍSTICAS");
    println!("Total de alunos: {}", total_average);
    println!("{:.2}% ({}) dos aluno(s) foram aprovados e \n{:.2}% ({}) dos aluno(s) estão de exame!", approved_percentage, students_approved, reproved_percentage, recovery_sum);
}

fn calc_percentage(current: i32, total: i32) -> f32 {
    let x = convert_to_f32(&current) / convert_to_f32(&total) * 100.0;
    x
}

fn convert_to_i32(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn convert_to_f32(data: & i32) -> f32 {
    let x = data.to_string().parse::<f32>().unwrap();
    x
}