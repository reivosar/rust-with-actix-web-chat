import { useState, useEffect, useRef } from 'react';
import './App.css';

const App = () => {

  type Message = {
    id: number;
    text: string;
    isUser: boolean;
  };

  const [messages, setMessages] = useState<Message[]>([]);
  const [inputMessage, setInputMessage] = useState('');
  const messageContainerRef = useRef<HTMLDivElement | null>(null); 
  const ws = useRef<WebSocket | null>(null);

  const handleMessageReceived = (event: MessageEvent) => {
    const message = event.data;
    setMessages(prevMessages => [...prevMessages, { text: message, isUser: false, id: Date.now() }]);
  };

  const handleUserMessage = () => {
    if (inputMessage.trim() !== '') {
      ws.current?.send(inputMessage);
      setMessages(prevMessages => [...prevMessages, { text: inputMessage, isUser: true, id: Date.now() }]);
      setInputMessage('');
    }
  };

  useEffect(() => {
    ws.current = new WebSocket('ws://localhost:3030');
    ws.current.onopen = () => {
      console.log('WebSocket connection opened');
    };
    ws.current.onmessage = handleMessageReceived;
    ws.current.onclose = () => {
      console.log('WebSocket connection closed');
    };

    return () => {
      ws.current?.close();
    };
  }, []); 

  useEffect(() => {
    if (messageContainerRef.current) {
      messageContainerRef.current.scrollTop = messageContainerRef.current.scrollHeight;
    }
  }, [messages.length]);

  return (
    <div className="min-h-screen bg-gray-200 flex items-center justify-center">
      <div className="bg-white shadow-md rounded-lg w-128">
        <div className="p-4">
          <div className="font-bold text-xl text-gray-800 mb-2">SampleChat</div>
          <div
            className="h-80 overflow-y-auto p-4 border-t border-b border-gray-300"
            ref={messageContainerRef}
          >
            {messages.map((message) => (
              <div
                key={message.id} 
                className={`mb-2 ${message.isUser ? 'text-right' : 'text-left'}`}
              >
                <div
                  className={`rounded-lg p-2 ${
                    message.isUser
                      ? 'bg-blue-400 text-white'
                      : 'bg-gray-300 text-gray-700'
                  }`}
                >
                  {message.text}
                </div>
              </div>
            ))}
          </div>
          <div className="flex mt-4">
            <input
              type="text"
              placeholder="Enter a message..."
              value={inputMessage}
              onChange={(e) => setInputMessage(e.target.value)}
              className="flex-grow rounded-l-lg border border-gray-300 p-2"
            />
            <button
              onClick={handleUserMessage}
              className="bg-blue-400 text-white p-2 rounded-r-lg"
            >
              Send
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default App;