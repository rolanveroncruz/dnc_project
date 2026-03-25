import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MasterListMemberComponent } from './master-list-member-component';

describe('MasterListMemberComponent', () => {
  let component: MasterListMemberComponent;
  let fixture: ComponentFixture<MasterListMemberComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MasterListMemberComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MasterListMemberComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
