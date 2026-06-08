import sys
import select

def get_message():
    if sys.stdin in select.select([sys.stdin], [], [], 0)[0]:
        raw_length = sys.stdin.buffer.read(4)
        message_length = struct.unpack('@I', raw_length)[0]
        message = sys.stdin.buffer.read(message_length).decode('utf-8')
        return message
    return None  

received_message = get_message()
if received_message:
    print(received_message)


