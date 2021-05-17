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

const client = new W3CWebSocket('ws://localhost:8080/');

export default class App extends Component {

  constructor(props) {
    super(props);

    this.state = {
      ready: false, // Identifica se servidor conectou ou nao
      connected: true, // Identifica se o servidor iniciou uma nova conexao
      client_id: null,
      input: "",
      chat: []
    }

    this.handleChange = this.handleChange.bind(this);
    this.send_message = this.send_message.bind(this);
    this.start_new_connection = this.start_new_connection.bind(this);
  }

  componentDidMount() {
    client.onopen = () => {
      console.log('WebSocket Client Connected');
      this.setState({ ready: true });
    };

    client.onmessage = (message) => {
      let data = JSON.parse(message.data);
      console.log(data);

      // If have recieved user_id
      if (data.user_id) {
        this.setState({ user_id: data.user_id })
      }

      // Diz ao react que uma nova conexao foi iniciada com um usuario aleatorio
      if (data.connected == true) {
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
      }

    };

  }

  send_message(command) {

    if (client.readyState === client.OPEN && command) {
      client.send(command);
      this.setState({ input: "" })
      return
    }

    if (client.readyState === client.OPEN && this.state.input != "") {
      client.send(this.state.input);
      this.setState({ input: "" })
    }
  }

  start_new_connection() {
    this.send_message("/new")
  }

  handleChange(event) {
    this.setState({ input: event.target.value })
  }

  render() {
    return (
      <div className="App">
        <header>
          <h1>Shuffle</h1>
          {
            this.state.ready ?
              <p>Pronto para iniciar</p> : <p>Conectando com o Servidor...</p>
          }
          <Button>
            <CloseIcon />
          </Button>
        </header>
        {/* Main chat */}
        <main>
          {this.state.chat.map((item) => {
            console.log(item.id)
            if (item.id === "shuffle") {
              return (<div className="chat-item shuffle">
                <p><strong>Shuffle </strong>{item.message}</p>
              </div>)
            } else if (item.id === "batata") {
              return (
                <div className="chat-item you">
                  <span><strong>Voce</strong></span>
                  <p>{item.message}</p>
                </div>
              )
            }
            else {
              return (
                <div className="chat-item">
                  <span><strong>Anonimo</strong></span>
                  <p>{item.message}</p>
                </div>
              )
            }
          })}

          {
            !this.state.connected && (
              <div>
                <button onClick={this.start_new_connection}>Clique aqui para <br /> Iniciar uma nova conversa</button>
              </div>
            )
          }
        </main>

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
