import { ComponentFixture, TestBed } from '@angular/core/testing';

import { GenericDataTableComponent } from './generic-data-table-component';

describe('GenericDataTableComponent', () => {
  let component: GenericDataTableComponent;
  let fixture: ComponentFixture<GenericDataTableComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [GenericDataTableComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(GenericDataTableComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
