import { TestBed } from '@angular/core/testing';

import { SearchDentistService } from './search-dentist-service';

describe('FindDentistService', () => {
  let service: SearchDentistService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SearchDentistService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
