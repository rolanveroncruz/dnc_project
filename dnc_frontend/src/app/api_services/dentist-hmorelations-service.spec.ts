import { TestBed } from '@angular/core/testing';

import { DentistHMORelationsService } from './dentist-hmorelations-service';

describe('DentistHMORelationsService', () => {
  let service: DentistHMORelationsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistHMORelationsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
