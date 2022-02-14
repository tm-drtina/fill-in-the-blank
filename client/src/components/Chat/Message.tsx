import { Theme } from '@mui/material';
import createStyles from '@mui/styles/createStyles';
import makeStyles from '@mui/styles/makeStyles';
import React from 'react';
import { Message } from '../../store/chat';
import Timestamp from './Timestamp';

type Props = {
  message: Message;
};

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    message: {
      display: 'flex',
      '& >span': {
        marginRight: '3px',
      },
      '& .username': {
        fontWeight: 'bold',
      },
    },
    systemMessage: {
      color: 'gray',
      '& .username': {
        fontStyle: 'italic',
      },
    },
  })
);

const MessageEl: React.FC<Props> = (props) => {
  const { message, systemMessage } = useStyles();
  const classes = props.message.systemMsg ? [message, systemMessage] : [message];
  return (
    <li className={classes.join(' ')}>
      <Timestamp timestamp={props.message.timestamp}/>
      <span className='username'>
        {props.message.username}:
      </span>
      <span>
        {props.message.content.split('\n').map((item, key) => <React.Fragment key={key}>{item}<br/></React.Fragment>)}
      </span>
    </li>
  );
};

export default MessageEl;
