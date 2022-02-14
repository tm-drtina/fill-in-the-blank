import { styled } from '@mui/material';
import React from 'react';
import { Message } from '../../store/chat';
import Timestamp from './Timestamp';

type Props = {
  message: Message;
};

const StyledMessage = styled('li')(() => ({
  display: 'flex',
  '>span': {
    marginRight: '3px',
  },
  '>.username': {
    fontWeight: 'bold',
  },
  '&.systemMessage': {
    color: 'gray',
    '& .username': {
      fontStyle: 'italic',
    },
  },
}));

const MessageEl: React.FC<Props> = (props) => {
  const classes = props.message.systemMsg ? 'systemMessage' : '';
  return (
    <StyledMessage className={classes}>
      <Timestamp timestamp={props.message.timestamp}/>
      <span className='username'>
        {props.message.username}:
      </span>
      <span>
        {props.message.content.split('\n').map((item, key) => <React.Fragment key={key}>{item}<br/></React.Fragment>)}
      </span>
    </StyledMessage>
  );
};

export default MessageEl;
