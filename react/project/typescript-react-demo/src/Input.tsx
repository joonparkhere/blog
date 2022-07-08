import React from "react";
import styled from "styled-components";

interface IInputProps {
    value: string;
    onChange: (event: React.SyntheticEvent<HTMLInputElement>) => void;
}

export const Input: React.FunctionComponent<IInputProps> = ({ value, onChange }) => (
    <input type="text" placeholder="Name" value={value} onChange={onChange} />
);

export const Form: React.FunctionComponent = ({ children }) => (
    <form>
        {children}
    </form>
);