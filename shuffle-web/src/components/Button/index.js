import React from 'react';
import { ButtonContainer } from './style';

export default function Button(props){
    return (
        <ButtonContainer onClick={props.onClick}>
            {props.children}
        </ButtonContainer>
    )
}