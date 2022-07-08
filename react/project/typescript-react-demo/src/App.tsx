import React, { Component } from 'react';
import Number from './Number';
import { Form, Input } from './Input';

interface IState {
  counter: number;
  name: string;
}

class App extends Component<{}, IState> {
  state = {
    counter: 0,
    name: ""
  };

  add = (): void => {
    this.setState(prev => {
      return {
        counter: prev.counter + 1
      }
    });
  }

  onChange = (event: React.SyntheticEvent<HTMLInputElement>): void => {
    
  }

  render() {
    const { counter, name } = this.state;
    return (
      <div>
        <Form>
          <Input value={name} onChange={this.onChange} />
        </Form>
        <Number count={counter} />
        <button onClick={this.add}>+</button>
      </div>
    );
  }
}

export default App;
