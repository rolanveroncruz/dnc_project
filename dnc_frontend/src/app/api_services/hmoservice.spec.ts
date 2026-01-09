import { TestBed } from '@angular/core/testing';

import { HMOService } from './hmoservice';

describe('HMOService', () => {
  let service: HMOService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HMOService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
