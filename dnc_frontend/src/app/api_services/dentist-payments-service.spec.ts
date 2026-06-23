import { TestBed } from '@angular/core/testing';

import { DentistPaymentsService } from './dentist-payments-service';

describe('DentistPaymentsService', () => {
  let service: DentistPaymentsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistPaymentsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
