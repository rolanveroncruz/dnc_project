import { TestBed } from '@angular/core/testing';

import { DentalClinicService } from './dental-clinic-service';

describe('DentalClinicService', () => {
  let service: DentalClinicService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentalClinicService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
