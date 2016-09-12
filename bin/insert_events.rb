# Push to a stream:
#   curl -i -d @event.txt "http://127.0.0.1:2113/streams/newstream" -H "Content-Type:application/vnd.eventstore.events+json"
# Retrieve a stream:
#   curl -i http://127.0.0.1:2113/streams/newstream/0/forward/20?embed=body  -H "Accept: application/json"
#

require 'securerandom'

EVENT_FILE_NAME = 'event.txt'

1000.times do |i|
    event = <<-JSON
[
  {
    "eventId": "#{SecureRandom.uuid}",
    "eventType": "event-type",
    "data": {

      "someDate": "#{Time.now}"
    }
  }
]
JSON

  File.open(EVENT_FILE_NAME, 'w') { |file| file.write event }

  command = %{curl -i -d @event.txt "http://127.0.0.1:2113/streams/newstream" -H "Content-Type:application/vnd.eventstore.events+json"}

  system command
end

File.delete EVENT_FILE_NAME
