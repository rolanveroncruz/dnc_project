import { TestBed } from '@angular/core/testing';

import { DentistClinicService } from './dentist-clinic-service';

describe('DentistClinicService', () => {
  let service: DentistClinicService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistClinicService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
