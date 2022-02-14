import { Box, Button, TextField } from '@mui/material';
import React, { KeyboardEvent, useState } from 'react';

import Wrapper from '../../components/Wrapper/Wrapper';
import { Message } from '../../store/chat';
import MessageEl from './Message';

type Props = {
  messages: Message[];
  sendMessage: (value: string) => void;
};

const Chat: React.FC<Props> = (props) => {
  const [formInput, formInputChange] = useState('');
  const sendMessage = () => {
    const trimmed = formInput.trim();
    if (trimmed.length > 0) {
      props.sendMessage(formInput);
      formInputChange('');
    }
  };
  const onSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    sendMessage();
    event.preventDefault();
  };
  const catchEnter = (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.key === 'Enter' && !event.shiftKey && !event.ctrlKey) {
      sendMessage();
      event.preventDefault();
    }
  };

  return (
    <>
      <Wrapper>
        <Box display="flex">
          <Box mt={12} display="flex" flexDirection="column" alignItems="flex-start" width={0.5}>
            <ul>
              {props.messages.map((message, index) =>
                <MessageEl key={index} message={message}/>,
              )}
            </ul>
            <form onSubmit={onSubmit}>
              <TextField
                autoFocus
                multiline
                aria-label="New message"
                placeholder="Message"
                value={formInput}
                onChange={event => formInputChange(event.target.value)}
                onKeyPress={catchEnter}
                label="Message"
              />
              <Button type='submit'>Send</Button>
            </form>
          </Box>
        </Box>
      </Wrapper>
    </>
  );
};

export default Chat;
