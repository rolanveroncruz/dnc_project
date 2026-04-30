import { TestBed } from '@angular/core/testing';

import { HMOBillingService } from './hmobilling-service';

describe('HMOBillingService', () => {
  let service: HMOBillingService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HMOBillingService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
