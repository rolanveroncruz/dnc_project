import { TestBed } from '@angular/core/testing';

import { DentistHmoServicesMatrixService } from './dentist-hmo-services-matrix-service';

describe('DentistHMOServicesMatrixService', () => {
  let service: DentistHmoServicesMatrixService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistHmoServicesMatrixService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
