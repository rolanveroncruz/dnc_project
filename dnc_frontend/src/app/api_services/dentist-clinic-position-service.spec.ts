import { TestBed } from '@angular/core/testing';

import { DentistClinicPositionService } from './dentist-clinic-position-service';

describe('DentistClinicPositionService', () => {
  let service: DentistClinicPositionService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistClinicPositionService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
