import { TestBed } from '@angular/core/testing';

import { EndorsementMasterListService } from './endorsement-master-list-service';

describe('EndorsementMasterListService', () => {
  let service: EndorsementMasterListService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(EndorsementMasterListService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
