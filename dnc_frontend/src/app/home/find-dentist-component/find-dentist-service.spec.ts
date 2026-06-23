import { TestBed } from '@angular/core/testing';

import { FindDentistService } from './find-dentist-service';

describe('FindDentistService', () => {
  let service: FindDentistService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(FindDentistService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
