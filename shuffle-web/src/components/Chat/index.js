import React from 'react';

export default function Chat(props){

    let itens = props.itens;

    return (
        <main>
          {itens.map((item, i) => {
            if (item.sender === "server") {
              return (<div className="chat-item shuffle" key={i}>
                <p><strong>Server </strong>{item.message}</p>
              </div>)
            } 
            else if (item.sender === props.you) {
              return (
                <div className="chat-item you" key={i}>
                  <span><strong>Voce</strong></span>
                  <p>{item.message}</p>
                </div>
              )
            }
            else {
              return (
                <div className="chat-item" key={i}>
                  <span><strong>Anonimo</strong></span>
                  <p>{item.message}</p>
                </div>
              )
            }
          }).reverse()}
        </main>
    )
}