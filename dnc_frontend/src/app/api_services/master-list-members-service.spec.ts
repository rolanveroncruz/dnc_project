import { TestBed } from '@angular/core/testing';

import { MasterListMembersService } from './master-list-members-service';

describe('MasterListMembersService', () => {
  let service: MasterListMembersService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MasterListMembersService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
