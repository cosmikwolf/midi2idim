// import * as fs from 'fs'

// var input_stream = fs.createReadStream("/tmp/midi")
let read_stream = process.stdin

const process_data = (data: Buffer) => {
    process.stdout.write("INPUT:" + data)
}

read_stream.on('data', process_data)
// input_stream.pipe()
// read_stream.setRawMode(false);

// // Checking if it is configured or not
// const status = read_stream.isRaw;

// process.stdout.write("status:" + status + "\n");

