//*********************************************************
//
// Shawn Fahimi
// Operating Systems
// Programming Project #4: Bankers
// April 7, 2023
// Instructor: Dr. Michael Scherger
//
// NOTE: This program is written in Rust, and is compiled
// using the Rust compiler. To run this program (assuming
// Rust is installed), navigate to the directory containing
// this file and run either of the following commands:
// 1. 'cargo build', then 'cargo run -- <input file>'
// OR
// 2. 'rustc banker.rs', then './banker <input file>'
//
// If using the first method, run 'cargo clean' to delete
// the 'target' directory after executing.
//
//*********************************************************

//*********************************************************
// Includes and Defines
//*********************************************************
use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

//*********************************************************
//
// Print Vector Function
//
// Prints the type of vector and the vector itself, with
// values labeled by the provided resource labels.
//
// Return Value
// ------------
// none
//
// Function Parameters
// -------------------
// vector      Vec<i32>   reference  vector to print
// name        &str       reference  type of vector (e.g. "Request", "Available", etc.)
// labels      Vec<char>  reference  resource labels for vector (e.g. ['A', 'B', 'C'])
// process_id  usize      value      process ID for request vector (only used for request vectors, default 0)
//
// Local Variables
// ---------------
// i    usize   loop counter
//
//*********************************************************
fn print_vector(vector: &Vec<i32>, name: &str, labels: &Vec<char>, process_id: usize) {

    //print the header message specifying the vector type
    eprintln!("The {} Vector is: ", name);

    //handle formatting for request vector
    if name == "Request" {
        eprint!("  ");
    }

    //print the resource labels
    for i in 0..labels.len() {

        //handle shifting/formatting for values above 9
        if vector[i] > 9 {
            eprint!(" ");
        }

        eprint!("{} ", labels[i]);
    }
    eprintln!("");

    //handle formatting for request vector
    if name == "Request" {
        eprint!("{}:", process_id);
    }

    //print the vector values
    for i in 0..vector.len() {
        eprint!("{} ", vector[i]);
    }
    eprintln!("\n");
}

//*********************************************************
//
// Print Matrix Function
//
// Prints the type of matrix and the matrix itself, with
// values labeled by the resource labels.
//
// Return Value
// ------------
// none
//
// Function Parameters
// -------------------
// matrix   Vec<Vec<i32>>  reference  matrix to print
// name     &str           reference  type of matrix (e.g. "Max", "Allocation", etc.)
// labels   Vec<char>      reference  resource labels for matrix (e.g. ['A', 'B', 'C'])
//
// Local Variables
// ---------------
// i       usize       loop counter used for both resource labels and rows in the matrix
// j       usize       loop counter used for both rows and columns in the matrix
// shift   Vec<bool>   used to indicate whether a matrix column needs to be shifted to the right (in case of values above 9)
//
//*********************************************************
fn print_matrix(matrix: &Vec<Vec<i32>>, name: &str, labels: &Vec<char>) {

    //used to determine whether matrix columns need to be shifted to the right (in case of values above 9)
    let mut shift: Vec<bool> = vec![false; labels.len()];

    //print the header message specifying the matrix type
    eprintln!("The {} Matrix is: ", name);
    eprint!("   ");

    //print the resource labels
    for i in 0..labels.len() {

        //handle shifting/formatting for values above 9
        for j in 0..matrix.len() {
            if matrix[j][i] > 9 {
                shift[i] = true;
                eprint!(" ");
                break;
            }
        }

        eprint!("{} ", labels[i]);
    }
    eprintln!("");

    //print the matrix, with each row labeled by the process number
    for i in 0..matrix.len() {
        eprint!("{}: ", i);
        for j in 0..matrix[i].len() {

            //handle formatting for values above 9 before printing the value
            if shift[j] && matrix[i][j] < 10 {
                eprint!(" ");
            }

            eprint!("{} ", matrix[i][j]);
        }
        eprintln!("");
    }
    eprintln!("");
}

//*********************************************************
//
// Read First Line Function
//
// Reads the first line of the file and returns the number
// of processes and the number of resources.
//
// Return Values (both returned as a tuple)
// ------------
// usize      number of processes
// usize      number of resources
//
// Function Parameters
// -------------------
// reader   BufReader<File>  reference  file reader
//
// Local Variables
// ---------------
// line           String     line read from file
// parts          Vec<&str>  vector of line parts containing the number of processes and resources
// num_processes  usize      number of processes
// num_resources  usize      number of resources
//
//*********************************************************
fn read_first_line(reader: &mut BufReader<File>) -> (usize, usize) {

    //initialize the line reader and read the first line in full
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    //initialize 'parts' to hold the line parts containing
    //the number of processes and resources
    let parts: Vec<&str> = line.trim().split(' ').collect();

    //read and store the number of processes and resources
    let num_processes = parts[0].parse().unwrap();
    let num_resources = parts[1].parse().unwrap();
    (num_processes, num_resources)
}

//*********************************************************
//
// Read Resource Vector Function
//
// Reads the resource vector from the file and returns it,
// along with the resource labels.
//
// Return Values (both returned as a tuple)
// ------------
// Vec<char>      vector of resource labels
// Vec<i32>       vector of resource values
//
// Function Parameters
// -------------------
// reader   BufReader<File>  reference  file reader
//
// Local Variables
// ---------------
// line             String     line read from file
// resource_labels  Vec<char>  vector of resource labels
// resource_vec     Vec<i32>   vector of resource values
// i                usize      used to create a resource label for each resource type
// part             &str       line part containing a resource value
// label            char       resource label to be stored in 'resource_labels'
//
//*********************************************************
fn read_resource_vector(reader: &mut BufReader<File>) -> (Vec<char>, Vec<i32>) {

    //initialize the line reader and skip over the blank line
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();

    //initialize the vectors that will stores resource labels and values
    let mut resource_labels = vec![];
    let mut resource_vec: Vec<i32> = vec![];

    //read the resource values, create a label for each resource type
    //and store them in their respective vectors
    //the resource labels are stored as characters, starting with 'A'
    for (i, part) in line.trim().split(' ').enumerate() {
        let label = ((i as u8) + b'A') as char;
        resource_labels.push(label);
        resource_vec.push(part.parse().unwrap());
    }
    (resource_labels, resource_vec)
}

//*********************************************************
//
// Read Available Vector Function
//
// Reads the available vector from the file and returns it.
//
// Return Value
// ------------
// Vec<i32>      vector of available resources
//
// Function Parameters
// -------------------
// reader   BufReader<File>  reference  file reader
//
// Local Variables
// ---------------
// line             String     line read from file
// available_vec    Vec<i32>   vector of available resources
// part             &str       used to parse each part of the line
//
//*********************************************************
fn read_available_vector(reader: &mut BufReader<File>) -> Vec<i32> {

    //initialize the line reader and skip over the blank line
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();

    //initialize the vector that will store the available resources
    let mut available_vec: Vec<i32> = vec![];

    //read the available resource values and store them in the vector
    for part in line.trim().split(' ') {
        available_vec.push(part.parse().unwrap());
    }
    available_vec
}

//*********************************************************
//
// Read Matrix Function
//
// Reads a matrix from the file and returns it.
//
// Return Value
// ------------
// Vec<Vec<i32>>      matrix of integers
//
// Function Parameters
// -------------------
// reader         BufReader<File>  reference  file reader
// num_processes  usize            value      number of processes
//
// Local Variables
// ---------------
// matrix    Vec<Vec<i32>>  matrix of integers, holds the matrix to be returned
// line      String         used to read lines/rows from the file
// row       Vec<i32>       used to process/store each row of the matrix
// part      &str           used to parse individual parts of each line
//
//************************************************************
fn read_matrix(reader: &mut BufReader<File>, num_processes: usize) -> Vec<Vec<i32>> {

    //initialize the matrix that stores our final result
    let mut matrix = vec![];

    //initialize the line reader and skip over the blank line
    let mut line: String = String::new();
    reader.read_line(&mut line).unwrap();

    //store each line in the file as a row in the matrix
    //stopping when we reach the number of processes
    for _ in 0..num_processes {

        //initialize the row vector to store the next line
        let mut row: Vec<i32> = vec![];

        //read the next line and parse it into a row of values
        //then store the row in the matrix
        line.clear();
        reader.read_line(&mut line).unwrap();

        //parse each part of the line into an integer and store it in the row
        for part in line.trim().split(' ') {
            row.push(part.parse().unwrap());
        }

        //add the row to the matrix
        matrix.push(row);
    }
    matrix
}

//*********************************************************
//
// Compute Need Matrix Function
//
// Computes the need matrix from the max matrix and the
// allocation matrix and returns it.
//
// Return Value
// ------------
// Vec<Vec<i32>>      need matrix
//
// Function Parameters
// -------------------
// num_processes      usize            value      number of processes
// num_resources      usize            value      number of resources
// max_matrix         Vec<Vec<i32>>    reference  max matrix
// allocation_matrix  Vec<Vec<i32>>    reference  allocation matrix
//
// Local Variables
// ---------------
// need_matrix  Vec<Vec<i32>>  used to store the need matrix to be returned
// i            usize          loop counter for processes
// j            usize          loop counter for resource types
// row          Vec<i32>       loop variable that is used to store each row of the need matrix
//
//*********************************************************
fn compute_need_matrix(num_processes: usize, num_resources: usize, max_matrix: &Vec<Vec<i32>>, allocation_matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    //initialize the matrix that stores our final result
    let mut need_matrix = vec![];

    //compute the need matrix, row by row (process by process)
    for i in 0..num_processes {

        //initialize a row of the need matrix for the current process
        let mut row = vec![];

        //compute and store the process need for each resource 
        //(max - allocation)
        for j in 0..num_resources {
            row.push(max_matrix[i][j] - allocation_matrix[i][j]);
        }

        //add the row to the need matrix
        need_matrix.push(row);
    }
    need_matrix
}

//*********************************************************
//
// Fulfill Request Function
//
// Determines if a resource request can be fulfilled and 
// if so, fulfills it.
//
// Return Value
// ------------
// bool    true if request can be fulfilled, false if not
//
// Function Parameters
// -------------------
// process_id        usize            value      process ID of the requesting process
// request_vec       Vec<i32>         reference  request vector
// num_resources     usize            value      number of resource types
// available_vec     Vec<i32>         reference  available vector
// allocation_matrix Vec<Vec<i32>>    reference  allocation matrix
// need_matrix       Vec<Vec<i32>>    reference  need matrix
//
// Local Variables
// ---------------
// i  usize  loop counter for resources
//
//*********************************************************
fn fulfill_request(process_id: usize, request_vec: &Vec<i32>, num_resources: usize, available_vec: &mut Vec<i32>, 
                    allocation_matrix: &mut Vec<Vec<i32>>, need_matrix: &mut Vec<Vec<i32>>) -> bool {

    //determine if the request can be fulfilled
    for i in 0..num_resources {

        //If the request exceeds the process need, this is in invalid 
        //request as the process cannot request more than it has declared
        //it needs
        if request_vec[i] > need_matrix[process_id][i] {
            return false;
        }

        //If the request exceeds the available resources, this is an
        //invalid request as the system does not have enough resources
        //to fulfill it
        if request_vec[i] > available_vec[i] {
            return false;
        }
    }

    //if we have made it to this part, fulfill the request
    //Refer to the slides detailing the Banker's Algorithm
    //for how requests are fulfilled
    for i in 0..num_resources {
        available_vec[i] -= request_vec[i];
        allocation_matrix[process_id][i] += request_vec[i];
        need_matrix[process_id][i] -= request_vec[i];
    }
    true
}

//*********************************************************
//
// Read Request Vector Function
//
// Reads the request vector from the file and returns the 
// requesting process id and the request vector as a tuple.
//
// Return Values (both returned as a tuple)
// ------------
// usize    the process id
// Vec<i32> the request vector
//
// Function Parameters
// -------------------
// reader   BufReader<File>  reference  file reader
//
// Local Variables
// ---------------
// line         String    stores the line read from the file
// request_vec  Vec<i32>  stores the request vector
// process_id   usize     stores the process id of the requesting process
// i            usize     loop counter for parts of the line
// part         &str      used to store each part of the line
//
//**********************************************************
fn read_request_vector(reader: &mut BufReader<File>) -> (usize, Vec<i32>) {

    //initialize the line reader and skip over the blank line
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();

    //initialize the vector that stores our final result
    //and the usize that stores the process id
    let mut request_vec: Vec<i32> = vec![];
    let mut process_id: usize = 0;

    //parse the line into the process id and the request vector
    for (i, part) in line.trim().split([':', ' ']).enumerate() {

        //the first part is the process id
        if i == 0 {
            process_id = part.parse().unwrap();
        }
        //the rest of the parts are the request vector values
        else {
            request_vec.push(part.parse().unwrap());
        }
    }
    (process_id, request_vec)
}

//*********************************************************
//
// Safe State Check Function
//
// Confirms whether the state indicated by the matrices
// 'max' and 'allocation', as well as the vector 'available' 
// is safe or not.
//
// Return Value
// ------------
// bool      true if safe state, false if not
//
// Function Parameters
// -------------------
// num_processes     usize          value      number of processes
// num_resources     usize          value      number of resource types
// max_matrix        Vec<Vec<i32>>  reference  max matrix
// allocation_matrix Vec<Vec<i32>>  reference  allocation matrix
// available_vec     Vec<i32>       reference  available vector
//
// Local Variables
// ---------------
// work_vec          Vec<i32>       work vector, used to track available resources
// finish_vec        Vec<bool>      finish vector, used to track which processes have finished
// safe_sequence     Vec<i32>       safe sequence, used to track the safe sequence of processes
// found             bool           used to track whether a process in the list that can finish has been found
// can_finish        bool           used to track whether an individual process can actually finish
// i                 usize          loop counter for processes
// j                 usize          loop counter for resources
//
//*********************************************************
fn is_safe_state(num_processes: usize, num_resources: usize, max_matrix: &Vec<Vec<i32>>, 
                    allocation_matrix: &Vec<Vec<i32>>, available_vec: &Vec<i32>) -> bool {

    //create work vector to keep track of available resources
    let mut work_vec = available_vec.clone();

    //create finish vector to keep track of which processes have finished
    let mut finish_vec = vec![false; num_processes];

    //create safe sequence vector to keep track of the safe sequence of processes
    let mut safe_sequence = vec![];

    //loop until all processes that can finish have finished
    loop {

        //used to track whether a process that can finish has been found
        //among those that haven't yet finished
        let mut found = false;

        //look for a process in the list that currently can finish 
        for i in 0..num_processes {

            //if the process hasn't finished, let's check if it can finish
            if finish_vec[i] == false {

                //used to track whether the process can finish
                let mut can_finish = true;

                //check if the process requirements can be met with the available resources
                for j in 0..num_resources {
                    if max_matrix[i][j] - allocation_matrix[i][j] > work_vec[j] {
                        can_finish = false;
                        break;
                    }
                }
                //if the resource requirements for the process can be met,
                //add the process to the safe sequence, add the resources
                //to the work vector, mark the process as finished and
                //mark that a process has been found that can finish
                if can_finish {
                    found = true;
                    finish_vec[i] = true;
                    for j in 0..num_resources {
                        work_vec[j] += allocation_matrix[i][j];
                    }
                    safe_sequence.push(i);
                    break;
                }
            }
        }
        //break out of the loop once no process has been found
        //that can finish
        if !found {
            break;
        }
    }
    //if the safe sequence contains all of the processes, we are in a safe state
    if safe_sequence.len() == num_processes {
        return true;
    }
    //otherwise, we are not in a safe state
    else{
        return false;
    }
} 

//*********************************************************
//
// Banker's Algorithm Function
//
// Runs the Banker's Algorithm on the given input file using
// the helper functions implemented above.
//
// Return Values
// ------------
// none
//
// Function Parameters
// -------------------
// reader            BufReader<File>  reference  file reader
// num_processes     usize            value      number of processes
// num_resources     usize            value      number of resources
//
// Local Variables
// ---------------
// resource_labels   Vec<String>     resource labels ('A', 'B', 'C', etc.)
// resource_vec      Vec<i32>        resource vector
// available_vec     Vec<i32>        available vector
// max_matrix        Vec<Vec<i32>>   max matrix
// allocation_matrix Vec<Vec<i32>>   allocation matrix
// need_matrix       Vec<Vec<i32>>   need matrix
// request_vec       Vec<i32>        request vector
// process_id        usize           process ID of the requesting process
// is_safe           bool            used to determine if a given or resulting state is safe
// is_valid          bool            used to determine if a request can be granted with the available resources
//
//**********************************************************
fn run_banker(reader: &mut BufReader<File>, num_processes: usize, num_resources: usize) {

    //print number of processes and resource types
    eprintln!("There are {0} processes and {1} resource types in the system.\n", num_processes, num_resources);

    //read and print resource labels and vector
    let (resource_labels, resource_vec) = read_resource_vector(reader);
    print_vector(&resource_vec, "Resource", &resource_labels, 0);
 
    //read and print available vector
    let mut available_vec = read_available_vector(reader);
    print_vector(&available_vec, "Available", &resource_labels, 0);
 
    //read and print max matrix
    let max_matrix = read_matrix(reader, num_processes);
    print_matrix(&max_matrix, "Max", &resource_labels);
 
    //read and print allocation matrix
    let mut allocation_matrix = read_matrix(reader, num_processes);
    print_matrix(&allocation_matrix, "Allocation", &resource_labels);
 
    //compute and print need matrix
    let mut need_matrix = compute_need_matrix(num_processes, num_resources, &max_matrix, &allocation_matrix);
    print_matrix(&need_matrix, "Need", &resource_labels);
 
    //determine if the current system is in safe state
    let is_safe = is_safe_state(num_processes, num_resources, &max_matrix, &allocation_matrix, &available_vec);
 
    //if the system is in a safe state, read the request vector and process id,
    //and determine if the request can be fulfilled
    if is_safe == true {
 
        eprintln!("THE SYSTEM IS IN A SAFE STATE.\n");
 
        //read and print the request vector
        let (process_id, request_vec) = read_request_vector(reader);
        print_vector(&request_vec, "Request", &resource_labels, process_id);
 
        //check if request is valid by simulating fulfillment of the request
        let is_valid = fulfill_request(process_id, &request_vec, num_resources, &mut available_vec, &mut allocation_matrix, &mut need_matrix);
 
        //if the request is valid, determine if the resulting state is safe
        if is_valid == true {

            let is_safe = is_safe_state(num_processes, num_resources, &max_matrix, &allocation_matrix, &available_vec);
 
            //if the resulting state is safe, print the new state
            if is_safe == true {
                eprintln!("THE REQUEST CAN BE GRANTED: NEW STATE FOLLOWS\n");
                print_vector(&resource_vec, "Resource", &resource_labels, 0);
                print_vector(&available_vec, "Available", &resource_labels, 0);
                print_matrix(&max_matrix, "Max", &resource_labels);
                print_matrix(&allocation_matrix, "Allocation", &resource_labels);
                print_matrix(&need_matrix, "Need", &resource_labels);
            }
            //if the resulting state is not safe, indicate so
            else {
                eprintln!("THE REQUEST CANNOT BE GRANTED.");
            } 
        }
        //if the request is not valid, indicate so
        else{
            eprintln!("THE REQUEST CANNOT BE GRANTED.");
        }  
    }
    //if the system is not in a safe state, indicate so
    else{
        eprintln!("THE SYSTEM IS NOT IN A SAFE STATE.");
    }
}

//*********************************************************
//
// Main Function
//
// Opens the file specified by the command line argument, 
// verifies that a valid number of resource types and 
// processes are entered on the first line, and
// calls the Banker's Algorithm function to operate on the
// remainder of the file. 
//
// Return Value
// ------------
// i32                exit code (0 if successful, 1 if not)
//
// Function Parameters
// -------------------
// none
//
// Local Variables
// ---------------
// args               Vec<String>     command line arguments
// file               File            input file
// reader             BufReader<File> file reader
// num_processes      usize           number of processes
// num_resources      usize           number of resources
//
//*********************************************************
fn main() {

    //collect arguments from command line
    let args: Vec<String> = env::args().collect();

    //check for correct usage/number of arguments
    if args.len() != 2{
        eprintln!("Usage: ./banker <input file> OR cargo run -- <input file>");
        process::exit(1);
    }

    //check that a valid input filepath was entered in the arguments
    if Path::new(&args[1]).exists() == false {
        eprintln!("Error: File does not exist");
        process::exit(1);
    }

    //open input file and the file reader
    let file = File::open(&args[1]).unwrap();
    let mut reader = BufReader::new(file);
    
    //read first line (number of processes and resource types)
    let (num_processes, num_resources) = read_first_line(&mut reader);

    //check that the number of processes and resources is valid
    if num_processes > 1024{
        eprintln!("Error: Number of processes must be less than or equal to 1024");
        process::exit(1);
    }
    if num_resources > 26{
        eprintln!("Error: Number of resource types must be less than or equal to 26");
        process::exit(1);
    }

    //run banker's algorithm and exit program on completion
    run_banker(&mut reader, num_processes, num_resources);
    process::exit(0);
}