import { TestBed } from '@angular/core/testing';

import { DentalServiceTypesService } from './dental-service-types-service';

describe('DentalServiceTypesService', () => {
  let service: DentalServiceTypesService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentalServiceTypesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
