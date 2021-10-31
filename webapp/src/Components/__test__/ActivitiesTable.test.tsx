import { cleanup, render } from '@testing-library/react';
import React from 'react';
import { ReactNode } from 'react';
import ActivitiesTable from "../ActivitiesTable";
import activities from "./data/activities.json";
import balance from "./data/balance.json";
import tags from "./data/tags.json";

describe('ActivitiesTable', () => {
  let wrapper: ReactNode = render(
    <ActivitiesTable activities={activities} balance={balance} tags={tags} />
  );

  it('should match the snapshot', async () => {
    expect(wrapper).toMatchSnapshot()
  });


  afterEach(cleanup);
})