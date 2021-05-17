import styled from 'styled-components';

export const ButtonContainer = styled.button`
    width: 40px;
    height: 40px;

    display: flex;
    justify-content: center;
    align-items: center;

    border-radius: 50px;
    border: none;
    background-color: #7289da;

    &:hover {
        background-color: #ffffff;
        svg{
        width: 24px;
        height: 24px;
        fill: black
    }
    }

    svg{
        width: 24px;
        height: 24px;
        fill: white
    }
`;