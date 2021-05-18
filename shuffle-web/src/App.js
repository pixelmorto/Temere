import { w3cwebsocket as W3CWebSocket } from "websocket";

// Styles
import './assets/styles/global.scss';
import './assets/styles/header.scss';
import './assets/styles/main.scss';
import './assets/styles/footer.scss';

// Components
import Button from './components/Button';

// Icons
import { ReactComponent as SendIcon } from './assets/icons/send.svg';
import { ReactComponent as CloseIcon } from './assets/icons/close.svg';
import { Component } from "react";
import Chat from "./components/Chat";

const client = new W3CWebSocket('ws://localhost:8080/');

export default class App extends Component {

  constructor(props) {
    super(props);

    this.state = {
      id: null,
      wsConn: new W3CWebSocket('ws://localhost:8080/'),
      anonymous_id: null,
      chat: [],

      //
      ready: false, // Identifica se servidor conectou ou nao
      connected: true, // Identifica se o servidor iniciou uma nova conexao
      client_id: null,
      input: "",
    }

    this.handleChange = this.handleChange.bind(this);
    this.send_message = this.send_message.bind(this);
  }

  componentDidMount() {
    this.state.wsConn.onmessage = (message) => {

      let data = JSON.parse(message.data);
      console.log(data)


      switch (data.event) {
        case "connected":
          this.setState({ id: data.id });
          console.log("You received a random identification \nYour id is: " + this.state.id);
          break;
        case "inbox":
          let temp_chat = this.state.chat;
          temp_chat.push(data.data)
          this.setState({chat: temp_chat})
          break;
        default:
          break;
      }

      // Diz ao react que uma nova conexao foi iniciada com um usuario aleatorio
      if (data.connected) {
        this.setState({ connected: true });
      }

      // Diz ao React que a conexao com o usuario foi encerrada
      if (data.disconnect) {
        this.setState({ connected: false })
      }

      // Diz ao react que uma nova mensagem foi recebida
      if (data.inbox) {
        let chat = this.state.chat;
        chat.push(data.inbox);
        this.setState({ chat })

        console.log(this.state.client_id)
        console.log(data.inbox.id)
        if (this.state.client_id == data.inbox.id) {
          console.log(true)
        } else { console.log(false) }
      }

    };

  }

  send_message() {
    if (client.readyState === client.OPEN && this.state.input != "") {
      let value = this.state.input;
      if (value.substring(0, 1) === "/") {
        this.state.wsConn.send(JSON.stringify({ event: "command", data: value, metadata: "" }));
        this.setState({ input: "" })
        return
      } else {
        this.state.wsConn.send(JSON.stringify({ event: "message", data: value, metadata: "" }));
        this.setState({ input: "" })
        return;
      }
    }
  }

  send_command(command) {
    if (client.readyState === client.OPEN && this.state.input != "") {
      this.state.wsConn.send(JSON.stringify({ event: "message", data: command, metadata: "" }));
      return;
    }
  }

  handleChange(event) {
    this.setState({ input: event.target.value })
  }

  render() {
    return (
      <div className="App">
        <header>
          <p>Oi</p>
          <h1>Shuffle</h1>
          <Button>
            <CloseIcon />
          </Button>
        </header>
        {/* Main chat */}
        <Chat itens={this.state.chat} you={this.state.id}/>

        {/* Footer */}
        <footer>
          <div className="section one-section">
          </div>
          <div className="section two-section">
            {/*If you not connected in a room you cant send message*/}
            {this.state.connected ?
              <input type="text" placeholder="Digite sua mensagem" value={this.state.input} onChange={this.handleChange} /> :
              <input type="text" placeholder="Voce precisa iniciar um chat para mandar mensagens..." value={this.state.input} disabled />
            }
          </div>
          <div className="section tree-section">
            <Button onClick={() => this.send_message()}>
              <SendIcon />
            </Button>
          </div>
        </footer>
      </div>
    );
  }
}
