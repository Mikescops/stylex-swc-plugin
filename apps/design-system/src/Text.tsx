import React, { type ReactNode } from 'react';
import * as stylex from '@stylexjs/stylex';
import { tokens } from './tokens.stylex';

const styles = stylex.create({
  text: {
    color: tokens.pink7,
  },
});

export interface TextProps {
  children: ReactNode;
}

export function Text({ children }: TextProps) {
  return <span {...stylex.props(styles.text)}>{children}</span>;
}
