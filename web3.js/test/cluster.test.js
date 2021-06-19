// @flow

import {expect} from 'chai';

import {clusterApiUrl} from '../src/util/cluster';

describe('Cluster Util', () => {
  it('invalid', () => {
    expect(() => {
      // $FlowExpectedError
      clusterApiUrl('abc123');
    }).to.throw();
  });

  it('devnet', () => {
    expect(clusterApiUrl()).to.eq('https://api.devnet.safecoin.org');
    expect(clusterApiUrl('devnet')).to.eq('https://api.devnet.safecoin.org');
    expect(clusterApiUrl('devnet', true)).to.eq(
      'https://api.devnet.safecoin.org',
    );
    expect(clusterApiUrl('devnet', false)).to.eq(
      'http://api.devnet.safecoin.org',
    );
  });
});
