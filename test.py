import subprocess
from confluent_kafka import Consumer

def main() :
    # Compile the Verilog file
    subprocess.run(["iverilog", "-o", "test_vcd_file", "counter.v", "counter_tb.v"])

    config = {
    'bootstrap.servers': 'localhost:9092',
    'group.id': 'my-group',
    'auto.offset.reset': 'earliest'
    }

    consumer = Consumer(config)
    consumer.subscribe(['vcd-topic'])

    while True:
        msg = consumer.poll(1.0)
        if msg is not None:
            print(f"Received message: {msg.value()}")


    # Run the compiled output
    subprocess.run(["vvp", "test_vcd_file"])

main()
