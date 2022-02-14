import React from 'react';
import Modal from '@mui/material/Modal';
import { styled } from '@mui/material';

interface IProps {
  open: boolean;
  onClose: () => void;
  children: React.ReactNode;
}

const ModalBody = styled('div')(({theme}) => ({
  position: 'absolute',
  top: '50%',
  left: '50%',
  transform: 'translate(-50%, -50%)',
  width: 400,
  backgroundColor: theme.palette.background.paper,
  boxShadow: theme.shadows[5],
  padding: theme.spacing(2, 4, 3),
  '&:focus': {
    outline: 'none',
  },
}));

const Dialog = ({ open, onClose, children }: IProps) => (
  <Modal open={open} onClose={onClose}>
    <ModalBody>{children}</ModalBody>
  </Modal>
);

export default Dialog;
