import { cleanup, render } from '@testing-library/react';
import React, { ReactElement } from 'react';
import { ReactNode } from 'react';
import TagPatternList from "../TagPatternList";
import tagsPattern from "./data/tagspattern.json";

describe('TagPatternList', () => {
    let wrapper: ReactNode = render(
        <TagPatternList patterns={tagsPattern} />
    );

    it('should match the snapshot', async () => {
        expect(wrapper).toMatchSnapshot()
    });

    afterEach(cleanup);
})