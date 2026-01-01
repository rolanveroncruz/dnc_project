import { TestBed } from '@angular/core/testing';

import { DentalServicesService } from './dental-services-service';

describe('DentalServicesService', () => {
  let service: DentalServicesService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentalServicesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
