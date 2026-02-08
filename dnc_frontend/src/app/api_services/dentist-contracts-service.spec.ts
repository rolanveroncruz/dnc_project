import { TestBed } from '@angular/core/testing';

import { DentistContractsService } from './dentist-contracts-service';

describe('DentistContractsService', () => {
  let service: DentistContractsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistContractsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
