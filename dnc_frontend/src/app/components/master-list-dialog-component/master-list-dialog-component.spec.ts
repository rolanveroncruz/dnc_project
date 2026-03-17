import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MasterListDialogComponent } from './master-list-dialog-component';

describe('MasterListDialogComponent', () => {
  let component: MasterListDialogComponent;
  let fixture: ComponentFixture<MasterListDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MasterListDialogComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MasterListDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
