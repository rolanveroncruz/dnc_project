import { TestBed } from '@angular/core/testing';

import { CSREndorsementsService } from './csrendorsements-service';

describe('CSREndorsementsService', () => {
  let service: CSREndorsementsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(CSREndorsementsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
